import fs from "fs";
import path from "path";

const filePath = path.normalize(
  `${__dirname}/../..//input_example/01_not_quite_lisp.txt`
);

const getText = () => {
  try {
    const data = fs.readFileSync(filePath, "utf-8");
    return data;
  } catch (error) {
    console.log({ error });
  }
};

const dict = {
  "(": 1,
  ")": -1,
};

const partOne = () => {
  const text = getText();
  let sum = 0;

  // text?.split("").forEach((e) => {
  //   sum += dict[e as keyof typeof dict];
  // });

  if (text) {
    for (let char of text) {
      sum += dict[char as keyof typeof dict];
    }
  }

  console.log({ sum });
};

const partTwo = () => {
  const text = getText();
  if (text) {
    let sum = 0;
    let pos = 0;
    for (let i = 0; i < text.length; i++) {
      sum += dict[text[i] as (keyof typeof dict)];

      if (sum === -1) {
        pos = i + 1
        break;
      }
    }
    console.log({ sum, pos });
  }
};

partOne();
partTwo();
