import { getInputChallenge } from "../services/adventofcode";

function run() {
  partOne();
}

const partOne = () => {
  getInputChallenge({ year: 2022, day: 1 })
    .then((res) => {
      const data = res.data.split("\n");
      const result : number[]= []
      let sum = 0

      data.forEach( (e: string) => {
        if(e === ''){
          result.push(sum) 
          sum = 0
          return;
        } 
        sum += Number(e)
      });

      result.sort((a, b) => Number(b) - Number(a))
      const first = result[0]
      const total = first + result[1] + result[2] 
      console.log({first, total})

    })
    .catch((err) => console.log({ err }));
};

run();
