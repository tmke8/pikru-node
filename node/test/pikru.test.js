import { expect } from 'chai';
import { Pikru } from '../dist/pikru_wasm.js';

describe('Pikru', () => {
    describe('constructor', () => {
        it('should create instance without options', () => {
            const pikru = new Pikru();
            expect(pikru).to.be.instanceOf(Pikru);
        });

        it('should create instance with options', () => {
            const pikru = new Pikru({ cssVariables: true });
            expect(pikru).to.be.instanceOf(Pikru);
        });
    });

    describe('render', () => {
        it('should render simple diagram to SVG', () => {
            const pikru = new Pikru();
            const svg = pikru.render('box "Hello"');
            expect(svg).to.be.a('string');
            expect(svg).to.include('<svg');
            expect(svg).to.include('</svg>');
            expect(svg).to.include('Hello');
        });

        it('should render diagram with arrow', () => {
            const pikru = new Pikru();
            const svg = pikru.render('box "A" arrow box "B"');
            expect(svg).to.include('A');
            expect(svg).to.include('B');
        });

        it('should include CSS variables when option is set', () => {
            const pikru = new Pikru({ cssVariables: true });
            const svg = pikru.render('box "Hello"');
            expect(svg).to.include('--pik-');
        });

        it('should not include CSS variables by default', () => {
            const pikru = new Pikru();
            const svg = pikru.render('box "Hello"');
            expect(svg).to.not.include('--pik-');
        });

        it('should throw on invalid input', () => {
            const pikru = new Pikru();
            expect(() => pikru.render('invalid {')).to.throw();
        });
    });
});
