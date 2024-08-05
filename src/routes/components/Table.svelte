<script>
	import { onMount } from 'svelte';
	import { refreshToken, token, BASE_URL } from '../stores.js';
	import { get } from 'svelte/store';

	let data = [];
	let loading = true;
	let error = null;

	let searchTerm = '';
	let sortColumn = 'name';
	let sortDirection = 'asc';
	let currentPage = 1;
	let itemsPerPage = 10;
	let paginationOptions = [10, 50, 100, 200, 'All'];

	onMount(async () => {
		try {
			console.log('token : ' + get(token));

			const response = await fetch(
				'https://apigateway-cms.aha.id/cms/master_data/list_master_product?page=1&limit=&search=&sort=product_name asc',
				{
					headers: {
						Authorization: `Bearer ${get(token)}`,
						'Content-Type': 'application/json'
					}
				}
			);
			const jsonData = await response.json();
      console.log(jsonData.data[25]);
			data = jsonData.data.map(item => ({
				productName: item.gpro_name,
        companyName: item.gpri_company_name,
        skuInternal: item.sku_internal,
        skuEnternal: item.sku_external,
        skuBarcode: item.sku_barcode,
			}));
			loading = false;
		} catch (e) {
			error = e.message;
			loading = false;
		}
	});

	$: filteredData = data.filter((item) =>
		Object.values(item).some((value) =>
			value.toString().toLowerCase().includes(searchTerm.toLowerCase())
		)
	);

	$: sortedData = [...filteredData].sort((a, b) => {
		if (a[sortColumn] < b[sortColumn]) return sortDirection === 'asc' ? -1 : 1;
		if (a[sortColumn] > b[sortColumn]) return sortDirection === 'asc' ? 1 : -1;
		return 0;
	});

	$: actualItemsPerPage = itemsPerPage === 'All' ? sortedData.length : parseInt(itemsPerPage);

	$: paginatedData = sortedData.slice(
		(currentPage - 1) * actualItemsPerPage,
		currentPage * actualItemsPerPage
	);

	$: totalPages = Math.ceil(sortedData.length / actualItemsPerPage);

	function handleSort(column) {
		if (sortColumn === column) {
			sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
		} else {
			sortColumn = column;
			sortDirection = 'asc';
		}
	}

	function nextPage() {
		if (currentPage < totalPages) currentPage++;
	}

	function prevPage() {
		if (currentPage > 1) currentPage--;
	}

	function changeItemsPerPage(value) {
		itemsPerPage = value;
		currentPage = 1; // Reset to first page when changing items per page
	}
</script>

<div class="container mx-auto p-6">
	{#if loading}
		<p class="text-center text-xl">Loading...</p>
	{:else if error}
		<p class="text-center text-xl text-red-500">Error: {error}</p>
	{:else}
		<div class="mb-4 flex justify-between items-center">
			<input
				type="text"
				placeholder="Search..."
				bind:value={searchTerm}
				class="w-1/2 p-2 border border-gray-300 rounded"
			/>
			<div class="flex items-center">
				<span class="mr-2">Items per page:</span>
				<select
					bind:value={itemsPerPage}
					on:change={() => changeItemsPerPage(itemsPerPage)}
					class="p-2 border border-gray-300 rounded"
				>
					{#each paginationOptions as option}
						<option value={option}>{option}</option>
					{/each}
				</select>
			</div>
		</div>

		<div class="overflow-x-auto">
			<table class="min-w-full bg-white">
				<thead>
					<tr>
						{#each Object.keys(data[0]) as column}
							<th
								class="px-6 py-3 border-b border-gray-200 bg-gray-50 text-left text-xs leading-4 font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
								on:click={() => handleSort(column)}
							>
								{column}
								{#if sortColumn === column}
									<span class="ml-1">{sortDirection === 'asc' ? '↑' : '↓'}</span>
								{/if}
							</th>
						{/each}
					</tr>
				</thead>
				<tbody>
					{#each paginatedData as row}
						<tr class="hover:bg-gray-100">
							{#each Object.values(row) as cell}
								<td class="px-6 py-4 whitespace-nowrap border-b border-gray-200">
									{#if cell.startsWith('http')}
										<a href={cell} target="_blank" class="text-blue-500 hover:underline">{cell}</a>
									{:else}
										{cell}
									{/if}
								</td>
							{/each}
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<div class="mt-4 flex justify-between items-center">
			<div>
				Showing {(currentPage - 1) * actualItemsPerPage + 1} to {Math.min(
					currentPage * actualItemsPerPage,
					sortedData.length
				)} of {sortedData.length} entries
			</div>
			<div>
				<button
					on:click={prevPage}
					disabled={currentPage === 1}
					class="px-4 py-2 border border-gray-300 rounded-md mr-2 {currentPage === 1
						? 'opacity-50 cursor-not-allowed'
						: 'hover:bg-gray-100'}"
				>
					Previous
				</button>
				<button
					on:click={nextPage}
					disabled={currentPage === totalPages}
					class="px-4 py-2 border border-gray-300 rounded-md {currentPage === totalPages
						? 'opacity-50 cursor-not-allowed'
						: 'hover:bg-gray-100'}"
				>
					Next
				</button>
			</div>
		</div>
	{/if}
</div>
