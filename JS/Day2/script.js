const fs = require("fs");

const partOne = () => {
  let totalScore = 0;

  fs.readFile(
    "JS/Day2/adventofcode.com_2022_day_2_input.txt",
    "utf8",
    (err, theData) => {
      if (err) {
        console.error(err);
        return;
      }
      const data = theData.split(/\r?\n/);

      for (let i = 0; i < data.length - 1; i++) {
        let my = data[i].charCodeAt(2) - 87;
        let other = data[i].charCodeAt(0) - 64;

        if (my == other) totalScore += 3;
        else if (my - 1 == other % 3) totalScore += 6;

        totalScore += data[i].charCodeAt(2) - 87;
      }

      console.log(totalScore);
    }
  );
};

const partTwo = () => {
  fs.readFile(
    "JS/Day2/adventofcode.com_2022_day_2_input.txt",
    "utf8",
    (err, theData) => {
      if (err) {
        console.error(err);
        return;
      }
      const data = theData.split(/\r?\n/);

      let totalScore = 0;

      let x = 0;
      let y = 0;
      let z = 0;

      for (let i = 0; i < data.length - 1; i++) {
        let other = data[i].charCodeAt(0) - 64;

        // X L
        // Y Draw
        // Z Skulle vært W

        if (data[i][2] == "X") {
          x++;
          totalScore += 0;
          if (other == 1) totalScore += 3;
          else totalScore += other - 1;
        } else if (data[i][2] == "Y") {
          y++;
          totalScore += 3;
          totalScore += other;
        } else if (data[i][2] == "Z") {
          z++;
          totalScore += 6;
          if (other == 3) totalScore += 1;
          else totalScore += other + 1;
        }
      }

      console.log("total score: " + totalScore);
      console.log(x);
      console.log(y);
      console.log(z);
    }
  );
};

partTwo();
