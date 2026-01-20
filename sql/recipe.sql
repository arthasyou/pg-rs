INSERT INTO recipe (
    kind,
    deps,
    calc_key,
    arg_map,
    expr,
    metric_code,
    metric_name,
    unit,
    value_type,
    visualization,
    status,
    created_at
) VALUES (
    'derived',
    '[16,18]'::jsonb,
    'tyg_v1',
    '{
        "TG": 16,
        "GLU": 18
    }'::jsonb,
    '{ "text": "TyG = f(TG, GLU)" }'::jsonb,
    'TYG',
    '胰岛素抵抗指数（TyG）',
    NULL,
    'float',
    'line',
    'active',
    now()
);