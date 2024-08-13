// place files you want to import through the `$lib` alias in this folder.
import Color from 'color';
export interface MarkSpan {
	start: number;
	end: number;
}
export interface Mark {
	a: MarkSpan;
	b: MarkSpan;
	color: string;
}

const goldenRatioConjugate: number = 0.618033988749895;

// https://martin.ankerl.com/2009/12/09/how-to-create-random-colors-programmatically/
export class HslGenerator {
	rand: number;

	constructor() {
		// Not random, but looks nice
		// Replace with seeded random number gen if wanted
		this.rand = 0.3072;
	}

	generateColor(): string {
		const c = Color.hsl([this.rand * 360, 0.5 * 100, 0.7 * 100]);
		this.rand += goldenRatioConjugate;
		this.rand %= 1;
		console.log(c.rgb().hex());
		return c.rgb().hex();
	}
}

export interface Item {
	path: string;
	contents: string;
}

export interface PairData {
	a: Item;
	b: Item;
	marks: Mark[];
	lang: string;
}

export interface ColoredMarkSpan {
	span: MarkSpan;
	color: string;
}
