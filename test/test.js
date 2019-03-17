import { shuffle } from "../shufflr.js";
import { expect } from "chai";

describe("shufflr", function() {
  describe("#shuffle()", function() {
    it("should return empty array when parameter is not an array", function() {
      var notArray = { one: 1, two: 2, three: 3 };

      expect(shuffle(notArray)).to.be.an("array").that.is.empty;
    });

    it("should not modify parameter array and return a copy of it shuffled", function() {
      var list = [1, 2, 3, 4, 5, 6, 7, 8, 9];

      expect(shuffle(list)).to.be.not.equal(list);
    });
  });
});
