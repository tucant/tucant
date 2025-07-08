import pdfplumber
import json

def persist_to_file(file_name):
    def decorator(original_func):
        try:
            cache = json.load(open(file_name, 'r'))
        except (IOError, ValueError):
            cache = {}
        def new_func(param1, param2):
            if str(param1) not in cache:
                cache[str(param1)] = original_func(param1, param2)
                json.dump(cache, open(file_name, 'w'))
            return cache[str(param1)]
        return new_func
    return decorator

#@persist_to_file('cache.dat')
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
    #print(table.extract())
    #im = page.to_image(resolution=150)
    #im.draw_rects(rects)
    #im.debug_tablefinder(table_settings)
    #im.show()
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
        print(cropped_table)
        parsed_rows.append(cropped_table)
        # one cell is never a table
        #if page_idx == 5:
        #    im = cropped_page.to_image(resolution=150)
        #    im.debug_tablefinder()
        #    im.show()
    if parsed_rows[0] is None:
        output.append(parsed_rows)
    else:
        output[-1].extend(parsed_rows)

# pkill gwenview

if __name__ == "__main__":
    pdf = pdfplumber.open("/home/moritz/Downloads/2023_05_11_MHB_MSC_INF.pdf")
    try:
        output = json.load(open("stage1.json", 'r'))
    except (IOError, ValueError):
        output = []
    for page_idx in range(4, len(pdf.pages)):
        print(f"page {page_idx}")
        page = pdf.pages[page_idx]
        handle_page(output, page_idx, page)
    print(output)
    json.dump(output, open("stage1.json", 'w'))