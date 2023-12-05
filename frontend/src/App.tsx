import { useMutation, useQuery } from "@apollo/client";
import { gql } from './__generated__/gql';
import React from "react";
import { theme } from "@diamondlightsource/ui-components"
import { ChakraProvider, Alert, AlertIcon, AlertTitle, AlertDescription, Button, HStack, Select } from "@chakra-ui/react";
import { PaginationTable } from "./components/PaginationTable";
import { PinStatus } from "./__generated__/graphql";

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

const PIN_FRAGMENT = gql(`
fragment pin on LibraryPin {
  barcode,
  status
}
`)


const LIBRARY_PINS_FRAGMENT = gql(`
fragment pinPage on LibraryPinConnection {
  edges {
    cursor
    node {
      ...pin
      loopSize,
    }
  }
}

`)

export interface UpdatePinStatusProps {
  item: Record<string, any>
}

const UpdatePinStatus = ({item }: UpdatePinStatusProps) => {

  const [status, setStatus] = React.useState(item['status']);
  
  const handleStatusChange = (event) => {
      setStatus(event.target.value);
  }; 
      const [
          updateLibraryPinStatus,
          { error: mutationError }
        ] = useMutation(UPDATE_PIN_STATUS, {

          update(cache, {data: {updateLibraryPinStatus}}) {

            cache.writeFragment({
              fragment: PIN_FRAGMENT,
              data: updateLibraryPinStatus,
              fragmentName: "pin",
            });
    
            const libraryPins = cache.readFragment({
              id: "barcode",
              fragment: LIBRARY_PINS_FRAGMENT,
              fragmentName: 'pinPage'
            });
            if (libraryPins) {    
              const newEdges = [...libraryPins.edges, updateLibraryPinStatus]
              cache.writeFragment({
                id: "barcode",
                fragment: LIBRARY_PINS_FRAGMENT,
                fragmentName: 'pinPage',
                data: { ...libraryPins, edges: newEdges }
              });
            }
          }
        })
  
      return (
          <div>
              <form
                  onSubmit={e => {
                  e.preventDefault();
                  updateLibraryPinStatus({ variables: { barcode: item['barcode'], status: status } });
  
                  ;
                  }}
              >
                      <Select value={status} onChange={handleStatusChange}>
                        {Object.values(PinStatus).map((status) => (
                          <option value={status}>{status}</option>))}
                      </Select>
                  <button type="submit">Update Status</button>
              </form>
              {mutationError && <p>Error :( Please try again</p>}
          </div>
      )
  }

// Displays libraryPins query in table component. The table can load more data if required 
function DisplayPinInfo(): React.JSX.Element {
  const { loading, error, data, fetchMore } = useQuery(
    GET_INFO,
    {
      notifyOnNetworkStatusChange: true,
    });

  var loadingRows = loading ? 2 : 0

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
        <Button marginTop={'1em'}
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

export {GET_INFO}
export { UpdatePinStatus }
