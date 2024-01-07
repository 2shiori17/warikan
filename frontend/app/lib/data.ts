import { graphql } from '~/gql';

export const GetGroupDetailQuery = graphql(`
  query GetGroupDetail($id: ID!) {
    getGroup(id: $id) {
      id
      createdAt
      title
      participants {
        id
        name
      }
      payments {
        id
        createdAt
        title
        creditor {
          id
          name
        }
        debtors {
          id
          name
        }
      }
    }
  }
`);

export const GetGroupsByUserQuery = graphql(`
  query GetGroupsByUser {
    getGroupsByUser {
      id
      createdAt
      title
      participants {
        id
      }
    }
  }
`)
