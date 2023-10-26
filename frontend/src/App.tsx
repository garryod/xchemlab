/* eslint-disable no-template-curly-in-string */
import { useQuery } from "@apollo/client";
import { gql } from './__generated__/gql';
import React from "react";
import { theme } from "./styles/theme"
import { ChakraProvider, Alert, AlertIcon, AlertTitle, AlertDescription, Button, HStack, useColorMode } from "@chakra-ui/react";
import { Table } from "./components/Table";
import {Helmet} from 'react-helmet';
import { DarkModeSwitch } from 'react-toggle-dark-mode';
import { DarkModeButton } from "./components/DarkModeButton";

const GET_INFO = gql(`
query pinInfo ($after: String) {
  libraryPins(first: 2, after: $after) {
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

// Displays libraryPins query in table component. The table can load more data if required 
function DisplayPinInfo(): React.JSX.Element {
  const { loading, error, data, fetchMore } = useQuery(
    GET_INFO,
    {
      notifyOnNetworkStatusChange: true,
    });

  //const [isDarkMode, setDarkMode] = React.useState(false);

  /* const toggleDarkMode = (checked: boolean) => {
    setDarkMode(checked);
  };*/

  const { colorMode, toggleColorMode } = useColorMode()

  var loadingRows = loading ? 2 : 0
  var bgColour = colorMode === 'dark' ? "black" : "white"

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
    <div>
    <DarkModeButton
      aria-label='Search database'
      style={{ marginBottom: '1rem', marginLeft: '.25rem', marginTop: '.25rem', position: 'relative', left: '1875px'}}
      isDark={colorMode === 'dark'}
      onClick={toggleColorMode}
      variant={'unstyled'}
      colorScheme={colorMode === 'dark' ? '#a3a3a3' : 'white'}
    />
        <Helmet>
        <style>{`body { background-color: ${bgColour} }`}</style>
      </Helmet>
      <Table 
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
    </ChakraProvider>
  );
}
