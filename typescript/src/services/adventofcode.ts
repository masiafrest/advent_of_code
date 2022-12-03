import axios from "axios";
import dotenv from "dotenv";
dotenv.config();

const sessionKey = process.env.AOC_KEY;

export const getInputChallenge = ({
  year,
  day,
}: {
  year: number;
  day: number;
}) => {
  const url = `https://adventofcode.com/${year}/day/${day}/input`;
  return axios
    .get(url, { headers: { cookie: `session=${sessionKey}` } })
};
