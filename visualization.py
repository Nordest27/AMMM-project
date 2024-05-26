from shimoku import Client
import pandas as pd

def get_treated_dataframe(df):
    print("Hello")
    df = df.groupby("alpha").mean().reset_index()
    df = df.melt(id_vars="alpha", var_name="problem", value_name="result")
    df["problem"] = [f"p{i}" for i in range(20)]
    print(df.to_string())
    return df

random_fits = get_treated_dataframe(pd.read_csv("results/random_fits.csv"))
random_fits_times = get_treated_dataframe(pd.read_csv("results/random_fits_times.csv"))
random_corner_fits = get_treated_dataframe(pd.read_csv("results/random_corner_fits.csv"))
random_corner_fits_times = get_treated_dataframe(pd.read_csv("results/random_corner_fits_times.csv"))

optimal_results = pd.read_csv("results/cplex_results.csv")

shimoku_client = Client()

shimoku_client.set_workspace()

shimoku_client.set_menu_path("AMMM-project")


data = random_fits.copy()
data['all fits'] = 100*(optimal_results['result'] - data['result'])/optimal_results['result']
data['corner fits'] = 100*(optimal_results['result'] - random_corner_fits['result'])/optimal_results['result']

shimoku_client.plt.bar(
    data=data,
    option_modifications={
        'legend': {},
        'xAxis': {
            'name': 'problem',
            'nameLocation': 'center',
            'nameGap': 35,
            'type': 'category', 
            'data': '#set_data#',
            'axisLabel': {
                'rotate': 45
            },
        },
        'yAxis': {
            'name': '%diff with optimal average of 10',
            'nameLocation': 'center',
            'nameGap': 35,
        },    
        'toolbox': {}
    },
    x='problem', 
    variant='clean thin',
    y=['all fits', 'corner fits'],
    order=0
)
data = random_fits_times.copy()
data['all fits'] = data['result']
data['corner fits'] = random_corner_fits_times['result']
shimoku_client.plt.bar(
    data=data,
    option_modifications={
        'legend': {},
        'xAxis': {
            'name': 'problem',
            'nameLocation': 'center',
            'nameGap': 35,
            'type': 'category', 
            'data': '#set_data#',
            'axisLabel': {
                'rotate': 45
            },
        },
        'yAxis': {
            'name': 'time (ms)',
            'nameLocation': 'center',
            'nameGap': 35,
        },    
        'toolbox': {}
    },
    x='problem', 
    variant='clean thin',
    y=['all fits', 'corner fits'],
    order=1
)
