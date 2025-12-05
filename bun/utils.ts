import path from "path";

export const readData = async (fileName: string) => {
  const finalPath = path.join(import.meta.dir, fileName);
  const foo = Bun.file(finalPath);
  const data = await foo.text(); // contents as a string
  const rows = data.split("\n").filter(Boolean);

  return rows;
};
