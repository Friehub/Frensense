// SAFE alternative: no-referrer for maximum privacy
import helmet from 'helmet';

const app = express();
app.use(helmet.referrerPolicy({ policy: 'no-referrer' }));

// or use <meta> tag in HTML
// <meta name="referrer" content="no-referrer">
