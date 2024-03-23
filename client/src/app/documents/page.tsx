"use client";
import { useSearchParams } from "next/navigation";

const Documents = () => {
  const searchParams = useSearchParams();
  const id = searchParams.get("id");
  return <div>My Document: {id}</div>;
};

export default Documents;
