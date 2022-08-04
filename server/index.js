const path = require('path');

const chance = new require('chance')();

const express = require('express');
const app = express();
const port = 3000;

const posts = new Array(5).fill(null).map(() => ({
  id: chance.guid(),
  title: chance.sentence({ words: 5 }),
  body: new Array(5).fill(null).map(() => chance.paragraph({ sentences: 5 }))
}));

const static = express.static(path.resolve('../client/dist'));

app.get('/api/posts', (req, res) => {
  res.json(posts);
});

app.get('/api/posts/:id', (req, res) => {
  res.json(posts.find(p => p.id === req.params.id));
});

app.post('/api/posts', (req, res) => {
  //
});

app.use(static);
app.use('*', static)

app.listen(port, () => {
  console.log(`App: http://127.0.0.1:${port}`);
});
