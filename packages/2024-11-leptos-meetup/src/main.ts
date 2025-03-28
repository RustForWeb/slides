import 'reveal.js/dist/reveal.css';
import 'highlight.js/styles/github.css';
import './theme/theme.scss';

import Reveal from 'reveal.js';
import Highlight from 'reveal.js/plugin/highlight/highlight.esm.js';
import Markdown from 'reveal.js/plugin/markdown/markdown.esm.js';
import Notes from 'reveal.js/plugin/notes/notes.esm.js';

const deck = new Reveal({
    width: '100%',
    height: '100%',
    plugins: [Highlight, Markdown, Notes]
});

deck.initialize();
