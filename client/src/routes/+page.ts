type Users = {
  id: string;
  first_name: string;
  last_name: string;
  email: string;
  password: string;
}[];

export const load = async ({ fetch }) => {
  const response = await fetch("http://localhost:5000/api");

  if (!response.ok) {
    throw new Error(`HTTP error: ${response.status}`);
  }

  const users: Users = await response.json();

  return { users };
};
