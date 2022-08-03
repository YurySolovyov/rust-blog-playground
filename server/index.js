const path = require('path');

const chance = new require('chance')();

const express = require('express');
const app = express();
const port = 3000;

const posts = new Array(5).fill(null).map(() => ({
  id: chance.guid(),
  title: chance.sentence(),
  body: chance.paragraph({ sentences: 3}),
}));

app.get('/api/posts', (req, res) => {
  res.json(posts);
});

app.get('/api/posts/:id', (req, res) => {
  res.json(posts.find(p => p.id === req.params.id));
});

app.use(express.static(path.resolve('../client/dist')));

app.listen(port, () => {
  console.log(`App: http://127.0.0.1:${port}`);
});
