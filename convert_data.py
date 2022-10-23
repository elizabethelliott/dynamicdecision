import csv
from openpyxl import Workbook
from openpyxl.styles import Font
from pathlib import Path
import os
import re

demographics_multi = [
    'demographics_gender.csv',
    'demographics_race.csv'
]
demographics_text = [
    'demographics_age.csv'
]
dynamic_filename = re.compile(r'lie_truth_dynamic_([0-9]*)\.csv')
dichotomous_filename = re.compile(r'lie_truth_dichotomous_([0-9]*)\.csv')
dichotomous_legend = [
    'Lie',
    'Truth'
]

def extract_video_id(matcher, filename) -> int | None:
    video_id = None
    search = matcher.search(filename)

    if search is not None:
        video_id = int(search.groups()[0])

    return video_id

def setup_video_struct(videos: dict, video_id: int) -> dict:
    if 'videos' not in videos:
        videos['videos'] = {}

    if video_id not in videos['videos'].keys():
        videos['videos'][video_id] = {}

    return videos

def write_data_to_cells(worksheet, row, data, starting_column = 1, bold = False):
    column = starting_column
    for d in data:
        worksheet.cell(row, column, d)
        if bold:
            worksheet.cell(row, column).font = Font(bold=True)
        column += 1

def write_to_xlsx(participant, data):
    workbook = Workbook()

    # Write all demographics to first sheet
    demographics_worksheet = workbook.create_sheet('Demographics')

    write_data_to_cells(demographics_worksheet, 1, ('Age', data['demographics_age']))
    write_data_to_cells(demographics_worksheet, 2, ('Race', f"{data['demographics_race'][1].strip()} ({data['demographics_race'][0]})"))
    write_data_to_cells(demographics_worksheet, 3, ('Gender', f"{data['demographics_gender'][1].strip()} ({data['demographics_gender'][0]})"))

    counterbalance = False

    for v_id in sorted(data['videos'].keys()):
        counterbalance = data['videos'][v_id]['counterbalance']

        video_worksheet = workbook.create_sheet(f'Video {v_id+1}')

        write_data_to_cells(video_worksheet, 1, ('Video path', data['videos'][v_id]['video_filename']))

        # Dynamic data
        video_worksheet.cell(4, 1, "Dynamic Decisions")
        video_worksheet.cell(4, 1).font = Font(bold=True)
        write_data_to_cells(video_worksheet, 5, ('Timestamp', 'Interim decision', 'Velocity'), bold=True)

        current_row = 6
        for d in data['videos'][v_id]['dynamic_decisions']:
            write_data_to_cells(video_worksheet, current_row, d)
            current_row += 1

        current_row +=1
        write_data_to_cells(video_worksheet, current_row, ('Final timestamp', 'Final decision'), bold=True)

        current_row += 1
        write_data_to_cells(video_worksheet, current_row, data['videos'][v_id]['dynamic_final'])

        # Dichotomous data (only write if we actually have data)
        if 'dichotomous_final' in data['videos'][v_id]:
            video_worksheet.cell(4, 5, "Dichotomous Decisions")
            video_worksheet.cell(4, 5).font = Font(bold=True)
            write_data_to_cells(video_worksheet, 5, ('Timestamp', 'Interim decision'), starting_column=5, bold=True)

            current_row = 6
            for d in data['videos'][v_id]['dichotomous_decisions']:
                write_data_to_cells(video_worksheet, current_row, d, starting_column=5)
                current_row += 1

            current_row +=1
            write_data_to_cells(video_worksheet, current_row, ('Final timestamp', 'Final decision'), starting_column=5, bold=True)

            current_row += 1
            write_data_to_cells(video_worksheet, current_row, data['videos'][v_id]['dichotomous_final'], starting_column=5)

    write_data_to_cells(demographics_worksheet, 4, ('Counterbalance', counterbalance))

    # Write the workbook to disk
    if not Path('reports/').exists():
        Path('reports/').mkdir()

    workbook.remove(workbook['Sheet'])
    workbook.save(f'reports/Participant {participant}.xlxs')

if __name__ == '__main__':
    print('Converting data files into XLXS files...')

    subdirs = [d[0] for d in os.walk('output/')]

    for d in subdirs:
        split_path = d.split('/')

        if len(split_path) == 2 and len(split_path[1]) == 0:
            continue

        participant_id = int(split_path[1])
        videos = {}

        print(f'Converting participant data in "{d}"')

        # Extract all of the data from the participant files
        for f in os.listdir(d):
            if dynamic_filename.match(f):
                video_id = extract_video_id(dynamic_filename, f)

                if video_id is not None:
                    videos = setup_video_struct(videos, video_id)

                    with open(d + '/' + f, 'r') as csv_file:
                        csv_data = csv.DictReader(csv_file)

                        videos['videos'][video_id]['dynamic_decisions'] = [(int(d['timestamp']), int(d['value']), float(d['velocity'])) for d in csv_data if d['type'] == 'decision']
                        csv_file.seek(0)
                        videos['videos'][video_id]['dynamic_final'] = [(int(d['timestamp']), int(d['value'])) for d in csv_data if d['type'] == 'final'][0]
                        csv_file.seek(0)
                        videos['videos'][video_id]['video_filename'] = [d['value'] for d in csv_data if d['type'] == 'path'][0]
                        csv_file.seek(0)
                        videos['videos'][video_id]['counterbalance'] = [d['value'] == 'true' for d in csv_data if d['type'] == 'counterbalance'][0]
            elif dichotomous_filename.match(f):
                video_id = extract_video_id(dichotomous_filename, f)

                if video_id is not None:
                    videos = setup_video_struct(videos, video_id)

                    with open(d + '/' + f, 'r') as csv_file:
                        csv_data = csv.DictReader(csv_file)

                        videos['videos'][video_id]['dichotomous_decisions'] = [(int(d['timestamp']), int(d['value'])) for d in csv_data if d['type'] == 'decision']
                        csv_file.seek(0)
                        videos['videos'][video_id]['dichotomous_final'] = [(int(d['timestamp']), int(d['value'])) for d in csv_data if d['type'] == 'final'][0]

            for d_file in demographics_multi:
                if Path(d + '/' + d_file).exists():
                    with open(d + '/' + d_file, 'r') as csv_file:
                        csv_data = csv.DictReader(csv_file)
                        csv_obj = next(csv_data)
                        videos[d_file.split('.')[0]] = (int(csv_obj['index']), csv_obj['label'])

            for d_file in demographics_text:
                if Path(d + '/' + d_file).exists():
                    with open(d + '/' + d_file, 'r') as csv_file:
                        csv_data = csv.DictReader(csv_file)
                        csv_obj = next(csv_data)
                        videos[d_file.split('.')[0]] = csv_obj['text']

        write_to_xlsx(participant_id, videos)
