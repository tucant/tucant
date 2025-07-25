import { getDocument, OPS } from "pdfjs-dist/legacy/build/pdf.mjs";
import { writeFile } from 'node:fs/promises'
import path from "node:path";

// node module-handbook.js

const DrawOPS = {
    moveTo: 0,
    lineTo: 1,
    curveTo: 2,
    closePath: 3,
};

const OPS_INVERTED = Object.fromEntries(
    Object.entries(OPS).map(([key, value]) => [value, key])
);

const document = await getDocument({
    url: "/home/moritz/Downloads/2023_05_11_MHB_MSC_INF.pdf",
    standardFontDataUrl: path.join(
        import.meta.dirname,
        "node_modules/pdfjs-dist/standard_fonts/"
    ) + '/',
}).promise

for (let i = 1; i <= document.numPages; i++) {
    await handlePage(await document.getPage(i))
}

/**
 * 
 * @param {import("pdfjs-dist").PDFPageProxy} page 
 */
async function handlePage(page) {
    const height = page.view[3]
    let svg = `<?xml version="1.0" encoding="UTF-8"?>
    <svg xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink"
        version="1.1" baseProfile="full"
        width="${page.view[2]}mm" height="${height}mm"
        viewBox="0 0 ${page.view[2]} ${height}">
        <rect width="${page.view[2]}mm" height="${height}mm" fill="black" />`

    const opList = await page.getOperatorList();

    const [horizontal, vertical] = extractLines(opList);
    let mergedHorizontal = mergeLines(horizontal);
    const mergedVertical = mergeLines(vertical);

    for (const horizontalLine of mergedHorizontal) {
        svg += `<line y1="${height - horizontalLine[0]}" y2="${height - horizontalLine[0]}" x1="${horizontalLine[1]}" x2="${horizontalLine[2]}" stroke="white" />`
    }
    for (const verticalLine of mergedVertical) {
        svg += `<line x1="${verticalLine[0]}" x2="${verticalLine[0]}" y1="${height - verticalLine[1]}" y2="${height - verticalLine[2]}" stroke="white" />`
    }

    const textContent = await page.getTextContent();
    textContent.items.forEach(textItem => {
        // https://github.com/mozilla/pdf.js/blob/542514efbdbfa022f9e01b6f9a0348522829ad8e/src/display/text_layer.js#L336
        let tx = textItem.transform
        var style = textContent.styles[textItem.fontName];
        var fontSize = Math.sqrt((tx[2] * tx[2]) + (tx[3] * tx[3]));
        svg += `<text x="${tx[4]}" y="${height - tx[5]}" fill="white" font-family="${style.fontFamily}" font-size="${fontSize}">${textItem.str}</text>`
    })

    svg += `</svg>`
    await writeFile(`/tmp/test${page.pageNumber}.svg`, svg);

    if (mergedHorizontal.length === 0 && mergedVertical.length === 0) {
        console.log(`page with only text`) // , textContent.items
        return;
    }

    //console.log(mergedHorizontal[1][0])

    // there are a few places that span more than two pages

    // page 539 has the bug that the title does not span the full page
    // mergedHorizontal.find(a => a[2] - a[1] > 499)
    if (mergedHorizontal[1][0] >= 747) {
        console.log("Modulbeschreibung first page")
        mergedHorizontal = mergedHorizontal.filter(a => a[0] < 747)

        const largeHorizontalLines = mergedHorizontal.filter((a) => a[2] - a[1] > 484)

        for (let i = 0; i < largeHorizontalLines.length - 1; i++) {
            const top = largeHorizontalLines[i]
            const bottom = largeHorizontalLines[i + 1]

            // get text in area
            console.log(extractText(height, textContent, [top[1], top[0], bottom[2], bottom[0]]))
        }
    } else {
        console.log("following page")
    }

    // lines that have a difference of less than 1 are the same length but overlap with perpendicular lines
    //console.log(mergedHorizontal.map((a) => a[2] - a[1]))
}

/**
 * 
 * @param {number} height
 * @param {TextContent} textContent 
 * @param {[number, number, number, number]} rect top-left-x, top-left-y, bottom-right-x, bottom-right-y
 */
function extractText(height, textContent, rect) {
    let text = ""
    textContent.items.forEach(textItem => {
        let tx = textItem.transform
        var style = textContent.styles[textItem.fontName];
        var fontSize = Math.sqrt((tx[2] * tx[2]) + (tx[3] * tx[3]));

        const y = height - tx[5];
        const x = tx[4];

        if (rect[0] <= x && x <= rect[2] && rect[1] <= y && y <= rect[3]) {
            text += textItem.str;
        }
    })
    return text
}

/**
 * 
 * @param {import("pdfjs-dist/types/src/display/api").PDFOperatorList} opList 
 * @returns {[[number, number, number][], [number, number, number][]]}
 */
function extractLines(opList) {
    let horizontal = []
    let vertical = []
    let visible = true;

    for (let i = 0; i < opList.fnArray.length; i++) {
        const fnId = opList.fnArray[i];
        const args = opList.argsArray[i];
        const opName = OPS_INVERTED[fnId];

        if (opName === "setFillRGBColor") {
            let [color] = args;
            if (color === '#000000') {
                visible = true;
            } else {
                visible = false;
            }
        }
        if (opName === "constructPath") {
            if (!visible) {
                continue;
            }
            let [op, data, minMax] = args;
            if (op !== 23) {
                continue;
            }
            let [path] = data;
            if (!(path.length == 13 && path[0] === DrawOPS.moveTo && path[3] === DrawOPS.lineTo && path[6] === DrawOPS.lineTo && path[9] === DrawOPS.lineTo && path[12] === DrawOPS.closePath)) {
                continue;
            }
            const topLeftX = path[1];
            const topLeftY = path[2];
            const topRightX = path[4];
            const topRightY = path[5];
            const bottomRightX = path[7];
            const bottomRightY = path[8];
            const bottomLeftX = path[10];
            const bottomLeftY = path[11];
            if (!(topLeftX === bottomLeftX & topLeftY === topRightY && bottomRightX === topRightX && bottomLeftY === bottomRightY)) {
                continue;
            }
            if (bottomRightY - topLeftY < 0.5) {
                horizontal.push([(topLeftY + bottomRightY) / 2, topLeftX, bottomRightX])
            }
            if (bottomRightX - topLeftX < 0.5) {
                vertical.push([(topLeftX + bottomRightX) / 2, topLeftY, bottomRightY])
            }
        }
    }
    return [horizontal, vertical]
}

/**
 * 
 * @param {[number, number, number][]} lines 
 */
function mergeLines(lines) {
    // group by whether same position
    /** @type {Map<number, [number, number, number][]} */
    let groupedLines = Map.groupBy(lines, line => line[0]);
    return [...groupedLines].flatMap(([key, value]) => {
        // sort by start of line
        value.sort((a, b) => a[1] - b[1])
        let mergedLines = [value[0]]
        for (let i = 1; i < value.length; i++) {
            if (mergedLines[mergedLines.length - 1][2] - value[i][1] < 1) {
                mergedLines[mergedLines.length - 1][2] = Math.max(mergedLines[mergedLines.length - 1][2], value[i][2])
            } else {
                mergedLines.push(value[i])
            }
        }
        return mergedLines
    })
}