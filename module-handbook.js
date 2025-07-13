import { getDocument, OPS } from "pdfjs-dist/legacy/build/pdf.mjs";
import { writeFile } from 'node:fs/promises'

const DrawOPS = {
    moveTo: 0,
    lineTo: 1,
    curveTo: 2,
    closePath: 3,
};

const OPS_INVERTED = Object.fromEntries(
    Object.entries(OPS).map(([key, value]) => [value, key])
);

// node module-handbook.js

const document = await getDocument("/home/moritz/Downloads/2023_05_11_MHB_MSC_INF.pdf").promise
//console.log(document)

// https://github.com/mozilla/pdf.js/blob/master/src/display/api.js

for (let i = 1; i <= document.numPages; i++) {
    const page = await document.getPage(i)
    const height = page.view[3]
    let svg = `<?xml version="1.0" encoding="UTF-8"?>
    <svg xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink"
        version="1.1" baseProfile="full"
        width="${page.view[2]}mm" height="${height}mm"
        viewBox="0 0 ${page.view[2]} ${height}">
        <rect width="${page.view[2]}mm" height="${height}mm" fill="aliceblue" />`

    const opList = await page.getOperatorList();

    // Walk through operator list
    for (let i = 0; i < opList.fnArray.length; i++) {
        const fnId = opList.fnArray[i];
        const args = opList.argsArray[i];

        const opName = OPS_INVERTED[fnId];
        //console.log(`Operation: ${opName}`, args);
        // https://github.com/mozilla/pdf.js/blob/e0783cd07557134798e1fc882b043376bc8b8b6e/src/display/canvas.js#L1421
        if (opName === "constructPath") {
            let [op, data, minMax] = args;
            let [path] = data;
            let svgPath = "";
            for (let i = 0, ii = path.length; i < ii;) {
                switch (path[i++]) {
                    case DrawOPS.moveTo:
                        svgPath += `M ${path[i++]},${height - path[i++]} `;
                        break;
                    case DrawOPS.lineTo:
                        svgPath += `${path[i++]},${height - path[i++]} `;
                        break;
                    case DrawOPS.curveTo:
                        /*console.log(`bezierCurveTo ${path[i++]},
                            ${path[i++]},
                            ${path[i++]},
                            ${path[i++]},
                            ${path[i++]},
                            ${path[i++]}
                        `);*/
                        break;
                    case DrawOPS.closePath:
                        //console.log(`closePath`);
                        break;
                    default:
                        warn(`Unrecognized drawing path operator: ${path[i - 1]}`);
                        break;
                }
            }
            svg += `<path stroke="white" d="${svgPath}" />`
        }
    }

    const textContent = await page.getTextContent();
    textContent.items.forEach(textItem => {
        let tx = textItem.transform
        var style = textContent.styles[textItem.fontName];
        var fontSize = Math.sqrt((tx[2] * tx[2]) + (tx[3] * tx[3]));
        svg += `<text x="${tx[4]}" y="${height - tx[5]}" fill="white" font-family="${style.fontFamily}" font-size="${fontSize}">${textItem.str}</text>`
    })

    svg += `</svg>`
    await writeFile(`/tmp/test${i}.svg`, svg);
}
