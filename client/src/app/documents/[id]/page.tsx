export default function Page({ params }: { params: { id: string } }) {
  return <div>My Document: {params.id}</div>;
}
