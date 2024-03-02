"use client";
import { ButtonProps, Button as _Button } from "@material-tailwind/react";
import { PropsWithChildren } from "react";

type Props = {
  size: ButtonProps["size"];
  fullWidth: ButtonProps["fullWidth"];
  className: string;
};

export const Button = (props: PropsWithChildren<Props>) => {
  return (
    <_Button
      size={props.size}
      fullWidth={props.fullWidth}
      className={`bg-black-light hover:bg-black-super-light normal-case ${props.className}`}
    >
      {props.children}
    </_Button>
  );
};
