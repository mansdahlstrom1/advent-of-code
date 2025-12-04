export const readData = async (fileName: String) => {
  const foo = Bun.file(`${import.meta.dir}/input/${fileName}`);
  const data = await foo.text(); // contents as a string
  const rows = data.split("\n").filter(Boolean);

  return rows;
};
