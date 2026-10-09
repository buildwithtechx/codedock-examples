const port = Number(Deno.env.get("PORT") || 3000);

Deno.serve({ port, hostname: "0.0.0.0" }, () => {
  return Response.json({ message: "Hello from Codedock Deno Example!" });
});