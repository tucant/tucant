import pdfplumber

# pkill gwenview

pdf = pdfplumber.open("/home/moritz/Downloads/2023_05_11_MHB_MSC_INF.pdf")
for page_idx in range(4, len(pdf.pages)):
    print(f"page {page_idx}")
    page = pdf.pages[page_idx]
    if len(page.rects) == 0:
        print(f"skipping page {page_idx}")
        continue
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
    print(rects)
    table = page.find_table(table_settings)
    if table is None:
        continue
    #im = page.to_image(resolution=150)
    #im.draw_rects(rects)
    #im.debug_tablefinder(table_settings)
    #im.show()
    for row in table.rows:
        cropped_table_settings = dict(
            vertical_strategy="explicit",
            horizontal_strategy="explicit",
            explicit_vertical_lines=rects,
            explicit_horizontal_lines=rects
        )
        cropped_page = page.crop((row.bbox[0], row.bbox[1]-1, row.bbox[2], row.bbox[3]), strict = False)
        #im = cropped_page.to_image(resolution=150)
        #im.debug_tablefinder()
        print(cropped_page.extract_table())
    #im.show()
