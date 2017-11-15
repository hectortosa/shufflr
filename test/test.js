import Shufflr from '../shufflr.js';
var chai = require('chai');

describe('Shufflr', function() {
  describe('#shuffle()', function() {
    it('should return empty array when parameter is not an array', function() {
      var notArray = {"one": 1, "two": 2, "three": 3};

      chai.should().not.exist(Shufflr.shuffle(notArray));
    });
  });
});
