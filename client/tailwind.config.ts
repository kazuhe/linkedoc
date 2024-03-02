import type { Config } from "tailwindcss";
import withMT from "@material-tailwind/react/utils/withMT";

const config: Config = {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    colors: {
      white: {
        DEFAULT: "#ededed",
      },
      black: {
        "super-light": "#363636",
        light: "#282828",
        DEFAULT: "#1c1c1c",
        dark: "#161616",
      },
    },
  },
  plugins: [],
};

export default withMT(config);
