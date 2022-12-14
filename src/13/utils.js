const fs = require('fs');
const path = require('path');

const getPairs = (fileName) => {
  const file = fs.readFileSync(path.join(__dirname, fileName)).toString();
  const pair = file.split('\n\n').map((pair) => {
    return pair.split('\n').map((row) => {
      return row && JSON.parse(row);
    });
  });
  return pair;
};

module.exports = {
  getPairs,
};
