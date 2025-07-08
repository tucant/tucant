import pdfplumber
import json

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
        explicit_horizontal_lines=rects
    )
    table = page.find_table(table_settings)
    if table is None:
        return
    if page_idx == 544:
        #print(table.extract())
        im = page.to_image(resolution=150)
        im.draw_rects(rects)
        im.debug_tablefinder(table_settings)
        im.show()
    parsed_rows = []
    for row in table.rows:
        cropped_table_settings = dict(
            vertical_strategy="explicit",
            horizontal_strategy="explicit",
            explicit_vertical_lines=rects,
            explicit_horizontal_lines=rects
        )
        cropped_page = page.crop((row.bbox[0]-1, row.bbox[1]-1.0, row.bbox[2]+1, row.bbox[3]+1.0), strict = False)
        #im = cropped_page.to_image(resolution=150)
        #im.debug_tablefinder()
        cropped_table = cropped_page.extract_table()
        if cropped_table is None:
            cropped_table = [[cropped_page.extract_text()]]
        parsed_rows.append(cropped_table)
        # one cell is never a table
        #if page_idx == 544:
        #    im = cropped_page.to_image(resolution=150)
        #    im.debug_tablefinder()
        #    im.show()
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

    assert module[3][0][0] == "1"
    assert module[3][0][1] == "Kurse des Moduls"
    assert module[3][1][1].replace("\n", " ") == "Kurs Nr."
    assert module[3][1][2] == "Kursname"
    assert module[3][1][3] == "Arbeitsaufwand\n(CP)"
    assert module[3][1][4] == "Lehrform"
    assert module[3][1][5] == "SWS"

    for course in module[3][2:]:
        kurs_nr = course[1]
        print(course)

    #print(module_name)
    #print(modul_nr)
    #print(leistungspunkte)
    #print(arbeitsaufwand)
    #print(selbststudium)
    #print(moduldauer)
    #print(angebotsturnus)
    #print(modulverantwortliche_person)

if __name__ == "__main__":
    pdf = pdfplumber.open("/home/moritz/Downloads/2023_05_11_MHB_MSC_INF.pdf")
    handle_page([], 544, pdf.pages[544])
    try:
        output = json.load(open("stage1.json", 'r'))
    except (IOError, ValueError):
        output = []
        for page_idx in range(4, len(pdf.pages)):
            print(f"page {page_idx}")
            page = pdf.pages[page_idx]
            handle_page(output, page_idx, page)
        json.dump(output, open("stage1.json", 'w'))
    #print(output)
    for module in output:
        parse_module(module)