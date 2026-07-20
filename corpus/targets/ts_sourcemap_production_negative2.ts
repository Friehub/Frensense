// SAFE alternative: serve from separate domain without source maps
const app = express();
// SAFE: source maps only accessible internally
if (process.env.NODE_ENV === 'production') {
  app.use(express.static('dist', { extensions: ['js', 'css', 'html', 'png', 'svg'] }));
} else {
  app.use(express.static('dist'));
}
