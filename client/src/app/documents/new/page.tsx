"use client";
import useSWRMutation from "swr/mutation";

type CreateDocumentRequest = {
  path: string;
  title: string;
  description: string;
  tags: string[];
};

const createDocument = async (
  url: string,
  { arg }: { arg: CreateDocumentRequest }
) => {
  const result = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(arg),
  });
  return await result.json();
};

export default function Home() {
  // ドキュメント作成
  const { trigger: createDocumentTrigger, data: documentResult } =
    useSWRMutation("http://localhost:8080/documents", createDocument);

  return (
    <div>
      <button
        className="px-4 py-2 mt-5 rounded bg-zinc-200 text-zinc-900"
        onClick={() => {
          createDocumentTrigger({
            path: "path/to/document",
            title: "Document Title",
            description: "Document Description",
            tags: ["tag1", "tag2", "tag3"],
          });
        }}
      >
        ドキュメントを作成
      </button>
      <div>ドキュメントを作成の結果: {JSON.stringify(documentResult)}</div>
    </div>
  );
}
