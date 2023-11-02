/* eslint-disable no-template-curly-in-string */
import { useMutation, useQuery } from "@apollo/client";
import { gql } from './__generated__/gql';
import React from "react";
import { theme } from "@diamondlightsource/ui-components"
import { ChakraProvider, Alert, AlertIcon, AlertTitle, AlertDescription, Button, HStack } from "@chakra-ui/react";
import { PaginationTable } from "./components/PaginationTable";

const GET_INFO = gql(`
query pinInfo ($after: String) {
  libraryPins(cursor: {first: 2, after: $after}) {
    pageInfo {
      hasPreviousPage,
      hasNextPage,
      startCursor,
      endCursor
    },
    edges {
      cursor
      node {
        barcode,
        loopSize,
        status
      }
    }
  }
}
`);

const UPDATE_PIN_STATUS = gql(`
  mutation updatePinStatus($barcode: String!, $status: PinStatus!) {
    updateLibraryPinStatus(barcode: $barcode, status: $status) {
      barcode
      status
    }
  }
`);

function UpdatePin() {
  const { loading, error, data } = useQuery(
    GET_INFO,
    {
      notifyOnNetworkStatusChange: true,
    });
  const [
    updateLibraryPinStatus,
    { loading: mutationLoading, error: mutationError }
  ] = useMutation(UPDATE_PIN_STATUS);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error :(</p>;

  return data.libraryPins.edges.map((edge) => {
    let input;

    return (
      <div >
        <p></p>
        <form
          onSubmit={e => {
            e.preventDefault();
            updateLibraryPinStatus({ variables: { barcode: edge.node.barcode, status: input.value } });

            input.value = "";
          }}
        >
          <input
            ref={node => {
              input = node;
            }}
          />
          <button type="submit">Update Todo</button>
        </form>
        {mutationLoading && <p>Loading...</p>}
        {mutationError && <p>Error :( Please try again</p>}
      </div>
    );
  });
}

// Displays libraryPins query in table component. The table can load more data if required 
function DisplayPinInfo(): React.JSX.Element {
  const { loading, error, data, fetchMore } = useQuery(
    GET_INFO,
    {
      notifyOnNetworkStatusChange: true,
    });

  const { toggleColorMode } = useColorMode()

  var loadingRows = loading ? 2 : 0
  const bgColour = useColorModeValue('white', 'black')

  if (error) return (
    <Alert status='error'>
      <AlertIcon />
      <AlertTitle>{error.message}</AlertTitle>
      <AlertDescription>{error.extraInfo}</AlertDescription>
    </Alert>
  )

  const loadMore = () => {
    fetchMore({

      variables: {
        after: data.libraryPins.pageInfo.endCursor,
      },

      updateQuery: (previousQueryResult, { fetchMoreResult }) => {
        const newEdges = fetchMoreResult.libraryPins.edges;
        const pageInfo = fetchMoreResult.libraryPins.pageInfo;

        // if newEdges actually have items,
        return newEdges.length
          ? // return a reconstruction of the query result with updated values
            {
              ...previousQueryResult,

              libraryPins: {
                ...previousQueryResult.libraryPins,

                edges: [...previousQueryResult.libraryPins.edges, ...newEdges],

                pageInfo,
              },
            }
          : // else, return the previous result
            previousQueryResult;
      },
    });
  };

  return (
    <>
      <PaginationTable 
        headers={[
          {
            key: 'barcode',
            label: 'Barcode',
            skeletonWidth: 12
          },
          {
            key: 'loopSize',
            label: 'Loop Size',
            skeletonWidth: 3
          },
          {
            key: 'status',
            label: 'Status',
            skeletonWidth: 7
          }
        ]}
        data={data ? data.libraryPins.edges.map((edge) => edge.node): []}
        loadingRows={loadingRows}
        rowVariant={"diamondStriped"}
      />
      <HStack justify='center' width='100%'>
        <Button marginTop={'10px'}
          colorScheme='teal' 
          variant='outline' 
          onClick={loadMore} 
          isLoading={loadingRows !== 0} 
          loadingText='Loading' 
          isDisabled={data ? !data.libraryPins.pageInfo.hasNextPage : false}
        >
          Load More
        </Button>
      </HStack>
    </div>
    </>
  );
}

export default function App(): React.JSX.Element {
  return (
    <ChakraProvider theme={theme}>
      <DisplayPinInfo />
      <UpdatePin/>
    </ChakraProvider>
  );
}
