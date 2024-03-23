"use client";
import { useSearchParams } from "next/navigation";
import { useFetch } from "@/hooks/use-fetch";

const Documents = () => {
  const searchParams = useSearchParams();
  const id = searchParams.get("id");
  const { data, error, isLoading } = useFetch<Document[]>(`/documents/${id}`);

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
