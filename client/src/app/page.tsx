"use client";
import { useFetch } from "@/hooks/use-fetch";
import useSWRMutation from "swr/mutation";

const initialize = async (url: string) => {
  const result = await fetch(url, {
    method: "POST",
  });
  return await result.json();
};

const Home = () => {
  const { data: helloData, error, isLoading } = useFetch<Document[]>("/hello");

  // 初期化
  const { trigger, data } = useSWRMutation(
    "http://localhost:8080/initialize",
    initialize
  );

  if (error) return <div>Failed to load</div>;
  if (isLoading) return <div>Loading...</div>;

  return (
    <div>
      <div>return helloData: {JSON.stringify(helloData)}</div>
      <button
        className="px-4 py-2 mt-5 rounded bg-zinc-200 text-zinc-900"
        onClick={() => {
          trigger();
        }}
      >
        初期化
      </button>
      <div>初期化の結果: {JSON.stringify(data)}</div>
    </div>
  );
};

export default Home;
