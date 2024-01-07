
import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
  overwrite: true,
  schema: "http://localhost:8080",
  documents: "app/**/*.ts",
  generates: {
    "app/gql/": {
      preset: "client",
      plugins: []
    }
  }
};

export default config;
