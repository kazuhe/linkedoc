"use client";
import useSWR from "swr";
import useSWRMutation from "swr/mutation";

const fetcher = async (url: string) => {
  const result = await fetch(url);
  return result.json();
};

const initialize = async (url: string) => {
  const result = await fetch(url, {
    method: "POST",
  });
  return await result.json();
};

export default function Home() {
  const {
    data: helloData,
    error: helloError,
    isLoading: helloIsLoading,
  } = useSWR("http://localhost:8080/hello", fetcher);

  // 初期化
  const { trigger, data } = useSWRMutation(
    "http://localhost:8080/initialize",
    initialize
  );

  if (helloError) return <div>Failed to load</div>;
  if (helloIsLoading) return <div>Loading...</div>;

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
}
