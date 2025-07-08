import pdfplumber

with pdfplumber.open("/home/moritz/Downloads/2023_05_11_MHB_MSC_INF.pdf") as pdf:
    page = pdf.pages[4]
    maxwidth_rect = max(page.rects, key=lambda rect: rect["width"])
    print(maxwidth_rect)
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
    im = page.to_image(resolution=150)
    #im.draw_rects(rects)
    im.debug_tablefinder(table_settings)
    im.show()