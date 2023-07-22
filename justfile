format:
  cargo +nightly fmt && sleek --indent-spaces 4 --uppercase "{{justfile_directory()}}/website-backend/migrations/**/*.sql" 

docker_build_website_backend:
  {{justfile_directory()}}/website-backend/docker/docker_build.sh

docker_build_website_backend_ci:
  {{justfile_directory()}}/website-backend/docker/docker_build_ci.sh

docker_push_website_backend_ci:
  {{justfile_directory()}}/website-backend/docker/docker_push_ci.sh
