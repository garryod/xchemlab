import { useMutation } from "@apollo/client";
import React from "react";
import { gql } from "../__generated__/gql";


export interface UpdatePinStatusProps {
    item: Record<string, any>
 }

const UpdatePinStatus = ({item }: UpdatePinStatusProps) => {

    const UPDATE_PIN_STATUS = gql(`
        mutation updatePinStatus($barcode: String!, $status: PinStatus!) {
            updateLibraryPinStatus(barcode: $barcode, status: $status) {
                barcode
                status
            }
        }
    `);

    const [
        updateLibraryPinStatus,
        { loading: mutationLoading, error: mutationError }
      ] = useMutation(UPDATE_PIN_STATUS);

    let input;

    return (
        <div>
            <form
                onSubmit={e => {
                e.preventDefault();
                updateLibraryPinStatus({ variables: { barcode: item['barcode'], status: input.value } });

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
    )
}

export { UpdatePinStatus }