export async function genericFetch(
  url: string,
  input: unknown
): Promise<unknown> {
  const response = await fetch(url, {
    credentials: "include",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "x-csrf-protection": "tucant",
    },
    body: JSON.stringify(input),
  });
  if (!response.ok) {
    throw new Error(`${response.status} ${response.statusText}`);
  }
  return (await response.json()) as unknown;
}
