// Type: Library Code
// Description: A simple functional try-catch block for use in functional programming.
// input: a function to attempt, and a function to run on failure
// output: the result of the attempt, or the result of the failure function
export default function Try(attempt: () => any, failure: (e: any) => any) {
  try {
    return attempt();
  } catch (e) {
    return failure(e);
  }
}
