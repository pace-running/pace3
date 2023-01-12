// eslint-disable-next-line @typescript-eslint/no-var-requires
const path = require('path');

module.exports = {
  process(src, filename) {
    return {
      code: `module.exports = ${JSON.stringify(path.basename(filename))};`
    };
  }
};
