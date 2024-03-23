import useSWR from "swr";

export const useFetch = <T>(path: string) => {
  const fetcher = async (url: string) => {
    const result = await fetch(url);
    return result.json();
  };
  const { data, error, isLoading } = useSWR<T>(
    `http://localhost:8080${path}`,
    fetcher
  );

  return { data, error, isLoading };
};
