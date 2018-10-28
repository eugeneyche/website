import typescript from 'typescript';
import commonjs from 'rollup-plugin-commonjs';
import nodeResolve from 'rollup-plugin-node-resolve';
import replacePlugin from 'rollup-plugin-replace';
import typescriptPlugin from 'rollup-plugin-typescript';

const plugins = [
    nodeResolve(),
    replacePlugin({
        'process.env.NODE_ENV': JSON.stringify('development'),
    }),
    commonjs({
        include: 'node_modules/**',
        namedExports: {
            'node_modules/react/index.js': ['Children', 'Component', 'PropTypes', 'createElement'],
            'node_modules/react-dom/index.js': ['render']
        }
    }),
    typescriptPlugin({typescript, importHelpers: true}),
];

export default {
    input: './js/main.tsx',
    output: {
        file: './dist/js/bundle.js',
        format: 'iife',
    },
    plugins,
};
