/**
 * Fix SVG sizing by extracting viewBox dimensions and setting explicit width/height.
 * @param {string} svg - The SVG string
 * @returns {string} - The SVG with width and height attributes set
 */
export function fixSvgSize(svg) {
    const viewBoxMatch = svg.match(/viewBox="([^"]+)"/);
    if (!viewBoxMatch) {
        return svg;
    }

    const parts = viewBoxMatch[1].split(/\s+/);
    if (parts.length !== 4) {
        return svg;
    }

    const width = Math.ceil(parseFloat(parts[2]));
    const height = Math.ceil(parseFloat(parts[3]));

    // Insert width and height after the opening <svg tag
    return svg.replace(/<svg/, `<svg width="${width}px" height="${height}px"`);
}
