import pdfplumber
import json
import os

# pkill gwenview

def handle_page(output, page_idx, page):
    if len(page.rects) == 0:
        print(f"skipping page {page_idx}")
        return
    # maxwidth_rect = max(page.rects, key=lambda rect: rect["width"])
    rects = list(filter(lambda rect: rect["width"] < 499, page.rects))
    leftmost_rect = min(rects, key=lambda rect: rect["x0"])
    rightmost_rect = max(rects, key=lambda rect: rect["x1"])
    table_settings=dict(
        vertical_strategy="explicit",
        horizontal_strategy="explicit",
        explicit_vertical_lines=[leftmost_rect["x0"], rightmost_rect["x1"]],
        explicit_horizontal_lines=rects,
        intersection_tolerance=10,
        snap_tolerance=10,
    )
    table = page.find_table(table_settings)
    if table is None:
        return
    if page_idx == 5180:
        #print(table.extract())
        im = page.to_image(resolution=150)
        im.draw_rects(rects)
        im.debug_tablefinder(table_settings)
        im.show()
    parsed_rows = []
    for row in table.rows:
        cropped_page = page.crop((row.bbox[0]-1, row.bbox[1]-1.0, row.bbox[2]+1, row.bbox[3]+1.0), strict = False)
        #cropped_left_to_right = sorted(cropped_page.rects, key=lambda rect: rect["x0"])
        #print(cropped_left_to_right[1])
        rects = list(filter(lambda rect: rect["x1"] < 100 or rect["x0"] > 525, cropped_page.rects))
        cropped_table_settings = dict(
            intersection_tolerance=10,
            snap_tolerance=10,
            vertical_strategy="explicit",
            explicit_vertical_lines=rects,
            #horizontal_strategy="explicit",
            #explicit_horizontal_lines=cropped_page.rects,
        )
        print(cropped_table_settings)
        cropped_table = cropped_page.extract_table(cropped_table_settings)
        # one cell is never a table
        if cropped_table is None:
            cropped_table = [[cropped_page.extract_text()]]
        parsed_rows.append(cropped_table)
        #if page_idx == 3700:
        im = cropped_page.to_image(resolution=150)
        im.draw_rects(rects)
        #im.debug_tablefinder(cropped_table_settings)
        im.show()
        print(cropped_table)
    if parsed_rows[0][0][0].startswith("Modulname"):
        output.append(parsed_rows)
    else:
        output[-1].extend(parsed_rows)

def parse_module(module):
    module_name = module[0][0][0].lstrip("Modulname\n")
    modul_nr = module[1][0][0].lstrip("Modul Nr.\n").replace("\n", "")
    leistungspunkte = module[1][0][1].lstrip("Leistungspun\nkte\n").rstrip(" CP")
    arbeitsaufwand = module[1][0][2].lstrip("Arbeitsaufwand\n").rstrip(" h")
    selbststudium = module[1][0][3].lstrip("Selbststudium\n").rstrip(" h")
    moduldauer = module[1][0][4].lstrip("Moduldauer\n").rstrip(" Semester")
    angebotsturnus = module[1][0][5].lstrip("Angebotsturnus\n").replace("\n", " ")
    sprache = module[2][0][0].lstrip("Sprache\n")
    modulverantwortliche_person = module[2][0][1].lstrip("Modulverantwortliche Person\n").replace("\n", " ")

    print(module_name)
    print(module[3][1])
    assert module[3][0][0] == "1"
    assert module[3][0][1] == "Kurse des Moduls"
    assert module[3][1][1].replace("\n", " ") == "Kurs Nr."
    assert module[3][1][2] == "Kursname"
    assert module[3][1][3] == "Arbeitsaufwand\n(CP)"
    assert module[3][1][4] == "Lehrform"
    assert module[3][1][5] == "SWS"

    for course in module[3][2:]:
        if course == [None, '', '', '', '', '']:
            continue
        kurs_nr = course[1].replace("\n", "")
        #print(course)
        #print(kurs_nr)

    #print(module_name)
    #print(modul_nr)
    #print(leistungspunkte)
    #print(arbeitsaufwand)
    #print(selbststudium)
    #print(moduldauer)
    #print(angebotsturnus)
    #print(modulverantwortliche_person)

if __name__ == "__main__":
    os.system("pkill gwenview")
    pdf = pdfplumber.open("/home/moritz/Downloads/2023_05_11_MHB_MSC_INF.pdf")
    #handle_page([], 37, pdf.pages[37])
    #exit(0)
    try:
        output = json.load(open("stage1.json", 'r'))
    except (IOError, ValueError):
        output = []
        for page_idx in range(4, len(pdf.pages)):
            print(f"page {page_idx}")
            page = pdf.pages[page_idx]
            handle_page(output, page_idx, page)
        #json.dump(output, open("stage1.json", 'w'))
    #print(output)
    for module in output:
        parse_module(module)