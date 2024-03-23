"use client";
import useSWR from "swr";
import { useSearchParams } from "next/navigation";

const fetcher = async (url: string) => {
  const result = await fetch(url);
  return result.json();
};

const Documents = () => {
  const searchParams = useSearchParams();
  const id = searchParams.get("id");

  const { data, error, isLoading } = useSWR(
    `http://localhost:8080/documents/${id}`,
    fetcher
  );

  if (error) return <div>Failed to load</div>;
  if (isLoading) return <div>Loading...</div>;

  return (
    <div>
      <div>My Document: {id}</div>
      <div>{JSON.stringify(data)}</div>
    </div>
  );
};

export default Documents;
