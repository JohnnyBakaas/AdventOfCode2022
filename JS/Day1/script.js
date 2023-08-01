const fs = require("fs");

const partOne = () => {
  let temp = 0;

  let bigest = 0;

  fs.readFile(
    "JS/Day1/adventofcode.com_2022_day_1_input.txt",
    "utf8",
    (err, theData) => {
      if (err) {
        console.error(err);
        return;
      }

      const data = theData.split(/\r?\n/);

      for (let i = 0; i < data.length; i++) {
        if (data[i] == "") {
          if (temp > bigest) bigest = temp;
          temp = 0;
        } else {
          temp += Number(data[i]);
        }
      }
      console.log(bigest);
      return bigest;
    }
  );
};

const partTwo = () => {
  let temp = 0;

  let No1 = 0;
  let No2 = 0;
  let No3 = 0;

  fs.readFile(
    "JS/Day1/adventofcode.com_2022_day_1_input.txt",
    "utf8",
    (err, theData) => {
      if (err) {
        console.error(err);
        return;
      }

      const data = theData.split(/\r?\n/);

      for (let i = 0; i < data.length; i++) {
        if (data[i] == "") {
          if (temp > No1) {
            No3 = No2;
            No2 = No1;
            No1 = temp;
          } else if (temp > No2) {
            No3 = No2;
            No2 = temp;
          } else if (temp > No3) No3 = temp;

          temp = 0;
        } else {
          temp += Number(data[i]);
        }
      }
      console.log(No1);
      console.log(No2);
      console.log(No3);
      console.log(No1 + No2 + No3);
      return No1 + No2 + No3;
    }
  );
};
