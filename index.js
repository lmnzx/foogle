console.log("Querying /api/search");
fetch("/api/search", {
	method: "POST",
	headers: { "Content-Type": "text/plain" },
	body: "glsl function for linearly interpolation 34534",
}).then((response) => console.log(response.json()));
