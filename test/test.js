import { shuffle } from "../shufflr.js";
import { expect } from "chai";

describe("shufflr", function() {
  describe("#shuffle()", function() {
    it("should return empty array when parameter is not an array", function() {
      var notArray = { one: 1, two: 2, three: 3 };

      expect(shuffle(notArray)).to.be.an("array").that.is.empty;
    });
  });
});
