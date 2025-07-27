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

let modules = [];

let pages = []
for (let i = 1; i <= document.numPages; i++) {
    pages.push(await handlePage(await document.getPage(i)))
}
console.log("written")
for (let page of pages) {
    extractPage(page)
}
console.log(JSON.stringify(modules, null, 1))

/**
 * 
 * @param {import("pdfjs-dist").PDFPageProxy} page 
 * @returns {[number, import("pdfjs-dist/types/src/display/api").TextContent, [number, number, number][], [number, number, number][]]}
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

    const [horizontal, vertical] = extractLines(height, opList);
    let mergedHorizontal = mergeLines(horizontal);
    const mergedVertical = mergeLines(vertical);
    mergedHorizontal.sort((a, b) => a[0] - b[0])
    mergedVertical.sort((a, b) => a[0] - b[0])
    // TODO FIXME we could filter out some lines shorter than 1px

    for (const horizontalLine of mergedHorizontal) {
        svg += `<line y1="${horizontalLine[0]}" y2="${horizontalLine[0]}" x1="${horizontalLine[1]}" x2="${horizontalLine[2]}" stroke="white" />`
    }
    for (const verticalLine of mergedVertical) {
        svg += `<line x1="${verticalLine[0]}" x2="${verticalLine[0]}" y1="${verticalLine[1]}" y2="${verticalLine[2]}" stroke="white" />`
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

    return [page.pageNumber, height, textContent, mergedHorizontal, mergedVertical]
}

/**
 * 
 * @param {[number, number, import("pdfjs-dist/types/src/display/api").TextContent, [number, number, number][], [number, number, number][]]} param 
 * @returns 
 */
function extractPage(param) {
    let [pageNumber, height, textContent, mergedHorizontal, mergedVertical] = param
    if (pageNumber === 1) {
        return;
    }
    console.log("page", pageNumber)
    if (mergedHorizontal.length === 0 && mergedVertical.length === 0) {
        console.log(`page with only text`) // , textContent.items
        return;
    }

    //console.log(mergedHorizontal[1][0])

    // there are a few places that span more than two pages

    // page 539 has the bug that the title does not span the full page
    // mergedHorizontal.find(a => a[2] - a[1] > 499)

    // TODO check that vertical lines start below the two horizontal lines
    const topmostVertical = Math.min(...mergedVertical.map(a => a[1])) - 1

    if (mergedHorizontal[1][0] < topmostVertical) { // check y position
        mergedHorizontal = mergedHorizontal.filter(a => a[0] > mergedHorizontal[1][0])
        // page 48 is smaller
        // TODO find the largest lines, maybe later we need to find the one multiple lines in the same row that start to the leftmost and rightmost
        const maxLength = Math.max(...mergedHorizontal.map(a => a[2] - a[1]))
        const largeHorizontalLines = mergedHorizontal.filter((a) => a[2] - a[1] >= maxLength - 10) // page 551 has a much shorter line

        let module = {
            courses: []
        }

        // modulname
        {
            const top = largeHorizontalLines[0]
            const bottom = largeHorizontalLines[1]

            module.modulename = extractText(height, textContent, [top[1], top[0], bottom[2], bottom[0]]).substring("Modulname\n".length)
        }

        // all the info
        {
            const top = largeHorizontalLines[1]
            const bottom = largeHorizontalLines[2]

            // find the vertical lines that are intersecting here
            let intersectingVerticalLines = []
            for (let mergedVerticalLine of mergedVertical) {
                if (mergedVerticalLine[1] < top[0] + 1 && mergedVerticalLine[2] > bottom[0] - 1) {
                    intersectingVerticalLines.push(mergedVerticalLine)
                }
            }
            for (let i = 0; i < intersectingVerticalLines.length - 1; i++) {
                let info = extractText(height, textContent, [intersectingVerticalLines[i][0], top[0], intersectingVerticalLines[i + 1][0], bottom[0]])
                if (info.startsWith("Modul Nr.\n")) {
                    module.modulNr = info.substring("Modul Nr.\n".length)
                } else if (info.startsWith("Leistungspun\nkte\n")) {
                    module.leistungspunkte = info.substring("Leistungspun\nkte\n".length)
                } else if (info.startsWith("Arbeitsaufwand\n")) {
                    module.arbeitsaufwand = info.substring("Arbeitsaufwand\n".length)
                } else if (info.startsWith("Selbststudium\n")) {
                    module.selbstudium = info.substring("Selbststudium\n".length)
                } else if (info === "Moduldauer ") {

                } else if (info.startsWith("Moduldauer\n")) {
                    module.moduldauer = info.substring("Moduldauer\n".length)
                } else if (info.startsWith("Angebotsturnus\n")) {
                    module.angebotsturnus = info.substring("Angebotsturnus\n".length)
                } else {
                    throw JSON.stringify(info)
                }
            }
        }

        // language, modulverantwortliche person
        {
            const top = largeHorizontalLines[2]
            const bottom = largeHorizontalLines[3]

            // find the vertical lines that are intersecting here
            let intersectingVerticalLines = []
            for (let mergedVerticalLine of mergedVertical) {
                if (mergedVerticalLine[1] < top[0] + 1 && mergedVerticalLine[2] > bottom[0] - 1) {
                    intersectingVerticalLines.push(mergedVerticalLine)
                }
            }
            for (let i = 0; i < intersectingVerticalLines.length - 1; i++) {
                let info = extractText(height, textContent, [intersectingVerticalLines[i][0], top[0], intersectingVerticalLines[i + 1][0], bottom[0]])
                if (info.startsWith("Sprache\n")) {
                    module.sprache = info.substring("Sprache\n".length)
                } else if (info === "Modulverantwortliche Person") {

                } else if (info.startsWith("Modulverantwortliche Person\n")) {
                    module.modulverantwortlichePerson = info.substring("Modulverantwortliche Person\n".length)
                } else {
                    throw JSON.stringify(info)
                }
            }
        }


        // 1
        {
            // TODO FIXME the lines are not exactly on the same height
            const top = largeHorizontalLines[3]
            const bottom = largeHorizontalLines[4] // only large lines are taken into account

            // find the vertical lines that are intersecting here
            let intersectingVerticalLines = []
            for (let mergedVerticalLine of mergedVertical) {
                if (mergedVerticalLine[1] < top[0] + 1 && mergedVerticalLine[2] > bottom[0] - 1) {
                    intersectingVerticalLines.push(mergedVerticalLine)
                }
            }
            //console.log(`--------`)
            for (let i = 0; i < intersectingVerticalLines.length - 1; i++) {
                //console.log(extractText(height, textContent, [intersectingVerticalLines[i][0], top[0], intersectingVerticalLines[i + 1][0], bottom[0]]))
                //console.log("------------------------------------------------")
            }

            // TODO get all horizontal lines in that range (including upper and lower?)
            const subHorizontalLines = mergedHorizontal.filter((a) => top[0] <= a[0] && a[0] <= bottom[0])
            // split on these (remove left part with the 1)
            for (let i = 0; i < subHorizontalLines.length - 1; i++) {
                //console.log(extractText(height, textContent, [intersectingVerticalLines[1][0], subHorizontalLines[i][0], intersectingVerticalLines[2][0], subHorizontalLines[i + 1][0]]))
                //console.log("------------------------------------------------")

                // now do vertical lines again
                let innerIntersectingVerticalLines = []
                for (let mergedVerticalLine of mergedVertical) {
                    if (mergedVerticalLine[1] < subHorizontalLines[i][0] + 5 && mergedVerticalLine[2] > subHorizontalLines[i + 1][0] - 5) { // larger margin of error
                        innerIntersectingVerticalLines.push(mergedVerticalLine)
                    }
                }

                let results = []
                for (let j = 0; j < innerIntersectingVerticalLines.length - 1; j++) {
                    results.push(extractText(height, textContent, [innerIntersectingVerticalLines[j][0], subHorizontalLines[i][0], innerIntersectingVerticalLines[j + 1][0], subHorizontalLines[i + 1][0]]))
                }
                console.log(results)
                if (results.length === 2 && results[0].trim() === "1" && results[1] === "Kurse des Moduls") {
                    continue;
                }
                if (results.length === 6 && results[0] === "" && (results[1] === "Kurs\nNr." || results[1] === "Kurs Nr. ") && results[2] === "Kursname " && results[3] === "Arbeitsaufwand\n(CP)" && results[4] === "Lehrform " && results[5] === "SWS") {
                    continue;
                }
                if (i < 2) {
                    throw results
                }
                if (results.length != 6) {
                    throw new Error(JSON.stringify(results))
                }
                if (results[0] === "" && results[1] === "" && results[2] === "" && results[3] === "" && results[4] === "" && results[5] === "") {
                    continue;
                }
                let course = {
                    kursNr: results[1],
                    kursName: results[2],
                    arbeitsaufwand: results[3],
                    lehrform: results[4],
                    sws: results[5],
                }
                module.courses.push(course)
            }
            // then for each row handle the vertical split
        }

        console.log("------------------------------------------------")
        for (let i = 4; i < largeHorizontalLines.length - 1; i++) {
            const top = largeHorizontalLines[i]
            const bottom = largeHorizontalLines[i + 1]

            // get text in area
            console.log(extractText(height, textContent, [top[1], top[0], bottom[2], bottom[0]]))
            console.log("------------------------------------------------")
        }

        modules.push(module)
    } else {
        console.log("following page")

        // page 53 has short top line

        console.log("------------------------------------------------")
        for (let i = 0; i < mergedHorizontal.length - 1; i++) {
            const top = mergedHorizontal[i]
            const bottom = mergedHorizontal[i + 1]

            // get text in area
            console.log(extractText(height, textContent, [top[1], top[0], bottom[2], bottom[0]]))
            console.log("------------------------------------------------")

            // find the vertical lines that are intersecting here
            let intersectingVerticalLines = []
            for (let mergedVerticalLine of mergedVertical) {
                if (mergedVerticalLine[1] < top[0] + 1 && mergedVerticalLine[2] > bottom[0] - 1) {
                    intersectingVerticalLines.push(mergedVerticalLine)
                }
            }
            console.log(`--------`)
            for (let i = 0; i < intersectingVerticalLines.length - 1; i++) {
                console.log(extractText(height, textContent, [intersectingVerticalLines[i][0], top[0], intersectingVerticalLines[i + 1][0], bottom[0]]))
                console.log("------------------------------------------------")
            }
        }
    }

    // lines that have a difference of less than 1 are the same length but overlap with perpendicular lines
    //console.log(mergedHorizontal.map((a) => a[2] - a[1]))
}

/**
 * 
 * @param {number} height
 * @param {import("pdfjs-dist/types/src/display/api").TextContent} textContent 
 * @param {[number, number, number, number]} rect top-left-x, top-left-y, bottom-right-x, bottom-right-y
 */
function extractText(height, textContent, rect) {
    // https://github.com/mozilla/pdf.js/blob/341a0b6d477d2909fcb14bcbfdf0d2fd37406cb0/src/core/evaluator.js#L2966
    let text = ""
    let lastY = 0
    textContent.items.forEach(textItem => {
        let tx = textItem.transform

        const y = height - tx[5];
        const x = tx[4];

        if (rect[0] <= x && x <= rect[2] && rect[1] <= y && y <= rect[3]) {
            if (y != lastY) {
                if (lastY != 0) {
                    text += "\n";
                }
                lastY = y;
            }

            text += textItem.str;
        }
    })
    return text
}

/**
 * 
 * @param {number} height
 * @param {import("pdfjs-dist/types/src/display/api").PDFOperatorList} opList 
 * @returns {[[number, number, number][], [number, number, number][]]}
 */
function extractLines(height, opList) {
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
            let [op, data] = args;
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
                const y = height - (topLeftY + bottomRightY) / 2
                if (topLeftX <= bottomRightX) {
                    horizontal.push([y, topLeftX, bottomRightX])
                } else {
                    horizontal.push([y, bottomRightX, topLeftX])
                }
            }
            if (bottomRightX - topLeftX < 0.5) {
                const start = height - topLeftY
                const end = height - bottomRightY
                if (start <= end) {
                    vertical.push([(topLeftX + bottomRightX) / 2, start, end])
                } else {
                    vertical.push([(topLeftX + bottomRightX) / 2, end, start])
                }
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
    lines.sort((a, b) => a[0] - b[0])
    /** @type {[number, number, number][][]} */
    let groupedLines = lines.reduce((acc, value) => {
        if (acc.length > 0 && Math.abs(acc[acc.length - 1][0][0] - value[0]) < 1) {
            acc[acc.length - 1].push(value)
        } else {
            acc.push([value])
        }
        return acc
    }, []);
    return [...groupedLines].flatMap((value) => {
        // sort by start of line
        value.sort((a, b) => a[1] - b[1])
        let mergedLines = [value[0]]
        for (let i = 1; i < value.length; i++) {
            if (value[i][1] - 1 < mergedLines[mergedLines.length - 1][2]) {
                mergedLines[mergedLines.length - 1][2] = Math.max(mergedLines[mergedLines.length - 1][2], value[i][2])
            } else {
                mergedLines.push(value[i])
            }
        }
        return mergedLines
    })
}