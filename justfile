format:
  cargo +nightly fmt && sleek --indent-spaces 4 --uppercase "{{justfile_directory()}}/website-backend/migrations/**/*.sql" 
