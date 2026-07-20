// SAFE alternative: use re2 library (non-backtracking regex engine)
import RE2 from 're2';

const EMAIL_REGEX = new RE2(/^[a-zA-Z]+@example\.com$/);

app.post('/api/validate', (req, res) => {
  if (EMAIL_REGEX.test(req.body.input)) {
    return res.json({ valid: true });
  }
  res.json({ valid: false });
});
